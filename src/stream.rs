/// Converts internal standard SSE payload (`data: ...\n\n`) into NDJSON.
///
/// Gemini transport can consume NDJSON for final streaming output while
/// internal stream handling remains standard SSE.
pub fn sse_to_ndjson_stream(sse: &str) -> String {
    let mut rewriter = SseToNdjsonRewriter::default();
    let mut out = Vec::new();
    out.extend_from_slice(rewriter.push_chunk(sse.as_bytes()).as_slice());
    out.extend_from_slice(rewriter.finish().as_slice());
    String::from_utf8_lossy(out.as_slice()).into_owned()
}

/// Incremental SSE -> NDJSON converter.
///
/// Feed bytes via [`SseToNdjsonRewriter::push_chunk`], then call
/// [`SseToNdjsonRewriter::finish`] when upstream ends.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SseToNdjsonRewriter {
    pending: Vec<u8>,
}

impl SseToNdjsonRewriter {
    /// Pushes one upstream chunk and returns converted NDJSON bytes ready
    /// for downstream emission.
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Vec<u8> {
        self.pending.extend_from_slice(chunk);
        let mut out = Vec::new();

        while let Some(newline_index) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line = self.pending.drain(..=newline_index).collect::<Vec<u8>>();
            if line.last().copied() == Some(b'\n') {
                line.pop();
            }
            self.process_line(line.as_slice(), &mut out);
        }

        out
    }

    /// Flushes trailing buffered bytes (if any) at stream end.
    pub fn finish(&mut self) -> Vec<u8> {
        if self.pending.is_empty() {
            return Vec::new();
        }
        let line = std::mem::take(&mut self.pending);
        let mut out = Vec::new();
        self.process_line(line.as_slice(), &mut out);
        out
    }

    fn process_line(&self, raw_line: &[u8], out: &mut Vec<u8>) {
        let line = raw_line.strip_suffix(b"\r").unwrap_or(raw_line);
        let Some(payload) = line.strip_prefix(b"data:") else {
            return;
        };

        let payload = trim_ascii(payload);
        if payload.is_empty() || payload == b"[DONE]" {
            return;
        }

        out.extend_from_slice(payload);
        out.push(b'\n');
    }
}

fn trim_ascii(input: &[u8]) -> &[u8] {
    let start = input
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(input.len());
    let end = input
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map(|index| index + 1)
        .unwrap_or(start);
    &input[start..end]
}

// ---------------------------------------------------------------------------
// Generic newline-delimited splitters
// ---------------------------------------------------------------------------

/// Drains complete `\n`-terminated lines from `pending` into `out`,
/// stripping trailing `\r\n`.  Empty lines are skipped.
pub fn drain_lines(pending: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
    while let Some(pos) = pending.iter().position(|byte| *byte == b'\n') {
        let mut line = pending.drain(..=pos).collect::<Vec<u8>>();
        if line.last().copied() == Some(b'\n') {
            line.pop();
        }
        if line.last().copied() == Some(b'\r') {
            line.pop();
        }
        if !line.is_empty() {
            out.push(line);
        }
    }
}

/// Splits `bytes` into newline-delimited lines.  Complete lines go through
/// [`drain_lines`]; any trailing bytes without a terminating `\n` are also
/// emitted as a final element.
pub fn split_lines(bytes: &[u8], out: &mut Vec<Vec<u8>>) {
    if bytes.is_empty() {
        return;
    }
    let mut pending = bytes.to_vec();
    drain_lines(&mut pending, out);
    if !pending.is_empty() {
        out.push(pending);
    }
}

/// Convenience wrapper that returns owned lines instead of appending to a vec.
pub fn split_lines_owned(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    split_lines(bytes, &mut out);
    out
}
