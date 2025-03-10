use std::io::Write;

use rustc_hash::FxHashSet;
use swc_common::{sync::Lrc, BytePos, LineCol, SourceMap, Span};

use super::{Result, WriteJs};

///
/// -----
///
/// Ported from `createTextWriter` of the typescript compiler.
///
/// https://github.com/Microsoft/TypeScript/blob/45eaf42006/src/compiler/utilities.ts#L2548
pub struct JsWriter<'a, W: Write> {
    indent: usize,
    line_start: bool,
    line_count: usize,
    line_pos: usize,
    new_line: &'a str,
    srcmap: Option<&'a mut Vec<(BytePos, LineCol)>>,
    srcmap_done: FxHashSet<(BytePos, u32, u32)>,
    /// Used to avoid including whitespaces created by indention.
    pending_srcmap: Option<BytePos>,
    wr: W,
}

impl<'a, W: Write> JsWriter<'a, W> {
    pub fn new(
        _: Lrc<SourceMap>,
        new_line: &'a str,
        wr: W,
        srcmap: Option<&'a mut Vec<(BytePos, LineCol)>>,
    ) -> Self {
        JsWriter {
            indent: Default::default(),
            line_start: true,
            line_count: 0,
            line_pos: Default::default(),
            new_line,
            srcmap,
            wr,
            pending_srcmap: Default::default(),
            srcmap_done: Default::default(),
        }
    }

    #[inline]
    fn write_indent_string(&mut self) -> Result {
        const INDENT: &str = "    ";

        for _ in 0..self.indent {
            self.raw_write(INDENT)?;
        }

        Ok(())
    }

    #[inline]
    fn raw_write(&mut self, data: &str) -> Result {
        // #[cfg(debug_assertions)]
        // tracing::trace!("Write: `{}`", data);

        self.wr.write_all(data.as_bytes())?;
        if self.srcmap.is_some() {
            self.line_pos += data.chars().count();
        }

        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write(&mut self, span: Option<Span>, data: &str) -> Result {
        if !data.is_empty() {
            if self.line_start {
                self.write_indent_string()?;
                self.line_start = false;

                if let Some(pending) = self.pending_srcmap.take() {
                    self.srcmap(pending);
                }
            }

            if let Some(span) = span {
                self.srcmap(span.lo());
            }

            self.raw_write(data)?;

            if let Some(span) = span {
                self.srcmap(span.hi());
            }
        }

        Ok(())
    }

    #[inline]
    fn srcmap(&mut self, byte_pos: BytePos) {
        if byte_pos.is_dummy() && byte_pos != BytePos(u32::MAX) {
            return;
        }

        if let Some(ref mut srcmap) = self.srcmap {
            if self
                .srcmap_done
                .insert((byte_pos, self.line_count as _, self.line_pos as _))
            {
                let loc = LineCol {
                    line: self.line_count as _,
                    col: self.line_pos as _,
                };

                // #[cfg(debug_assertions)]
                // tracing::trace!("SourceMap: {:?} => {:?}", byte_pos, loc);

                srcmap.push((byte_pos, loc));
            }
        }
    }
}

impl<'a, W: Write> WriteJs for JsWriter<'a, W> {
    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn increase_indent(&mut self) -> Result {
        self.indent += 1;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn decrease_indent(&mut self) -> Result {
        self.indent -= 1;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_semi(&mut self, span: Option<Span>) -> Result {
        self.write(span, ";")?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_space(&mut self) -> Result {
        self.write(None, " ")?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_keyword(&mut self, span: Option<Span>, s: &'static str) -> Result {
        self.write(span, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_operator(&mut self, span: Option<Span>, s: &str) -> Result {
        self.write(span, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_param(&mut self, s: &str) -> Result {
        self.write(None, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_property(&mut self, s: &str) -> Result {
        self.write(None, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_line(&mut self) -> Result {
        let pending = self.pending_srcmap.take();
        if !self.line_start {
            self.raw_write(self.new_line)?;
            self.line_count += 1;
            self.line_pos = 0;
            self.line_start = true;

            if let Some(pending) = pending {
                self.srcmap(pending)
            }
        }

        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_lit(&mut self, span: Span, s: &str) -> Result {
        if !s.is_empty() {
            self.srcmap(span.lo());

            self.write(None, s)?;

            if self.srcmap.is_some() {
                let line_start_of_s = compute_line_starts(s);
                if line_start_of_s.len() > 1 {
                    self.line_count = self.line_count + line_start_of_s.len() - 1;
                    let last_line_byte_index = line_start_of_s.last().cloned().unwrap_or(0);
                    self.line_pos = s[last_line_byte_index..].chars().count();
                }
            }

            self.srcmap(span.hi());
        }

        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_comment(&mut self, s: &str) -> Result {
        self.write(None, s)?;
        if self.srcmap.is_some() {
            let line_start_of_s = compute_line_starts(s);
            if line_start_of_s.len() > 1 {
                self.line_count = self.line_count + line_start_of_s.len() - 1;
                let last_line_byte_index = line_start_of_s.last().cloned().unwrap_or(0);
                self.line_pos = s[last_line_byte_index..].chars().count();
            }
        }
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_str_lit(&mut self, span: Span, s: &str) -> Result {
        if !s.is_empty() {
            self.srcmap(span.lo());
            self.write(None, s)?;

            if self.srcmap.is_some() {
                let line_start_of_s = compute_line_starts(s);
                if line_start_of_s.len() > 1 {
                    self.line_count = self.line_count + line_start_of_s.len() - 1;
                    let last_line_byte_index = line_start_of_s.last().cloned().unwrap_or(0);
                    self.line_pos = s[last_line_byte_index..].chars().count();
                }
            }

            self.srcmap(span.hi());
        }

        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_str(&mut self, s: &str) -> Result {
        self.write(None, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_symbol(&mut self, span: Span, s: &str) -> Result {
        self.write(Some(span), s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn write_punct(&mut self, span: Option<Span>, s: &'static str) -> Result {
        self.write(span, s)?;
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn care_about_srcmap(&self) -> bool {
        self.srcmap.is_some()
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn add_srcmap(&mut self, pos: BytePos) -> Result {
        if self.line_start {
            self.pending_srcmap = Some(pos);
        } else {
            self.srcmap(pos);
        }
        Ok(())
    }

    #[inline]
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn commit_pending_semi(&mut self) -> Result {
        Ok(())
    }
}

fn compute_line_starts(s: &str) -> Vec<usize> {
    let mut res = vec![];

    let mut line_start = 0;

    let mut chars = s.char_indices().peekable();

    while let Some((pos, c)) = chars.next() {
        match c {
            '\r' => {
                if let Some(&(_, '\n')) = chars.peek() {
                    let _ = chars.next();
                }
            }

            '\n' => {
                res.push(line_start);
                line_start = pos + 1;
            }

            _ => {}
        }
    }

    // Last line.
    res.push(line_start);
    res
}
