use std::io::{Read, Result, Write};
use std::marker::PhantomData;

pub struct ReadStats<R> {
    _marker: ::std::marker::PhantomData<R>,
    reader: R,
    num_reads: usize,
    num_bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(reader: R) -> ReadStats<R> {
        ReadStats {
            _marker: PhantomData,
            reader,
            num_reads: 0,
            num_bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        unimplemented!("ReadStats get_ref")
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.reader.read(buf)?;
        self.num_reads += 1;
        self.num_bytes_read += n;
        Ok(n)
    }
}

pub struct WriteStats<W>(::std::marker::PhantomData<W>);

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats(PhantomData)
    }

    pub fn get_ref(&self) -> &W {
        unimplemented!("WriteStats get_ref")
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!("WriteStats bytes_through")
    }

    pub fn writes(&self) -> usize {
        unimplemented!("WriteStats writes")
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call writing {:?}", buf)
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!("WriteStats write")
    }
}
