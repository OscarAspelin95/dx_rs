pub enum AcceptFileTypes {
    Gz,
    Txt,
    Any,
    Fasta,
    Fastq,
}

impl AcceptFileTypes {
    // A note on file accept types. Generally, multiple extensions
    // such as "file.txt.gz" cannot be identified by the file picker.
    // In these cases, we'll need to use ".gz" and verify the entire
    // extension somewhere else.
    pub fn to_str(self) -> &'static str {
        match self {
            // Any gzip file.
            AcceptFileTypes::Gz => ".gz",
            // Any text file.
            AcceptFileTypes::Txt => ".txt",
            // Any file.
            AcceptFileTypes::Any => "",
            // Any plain FASTA file.
            AcceptFileTypes::Fasta => ".fasta, .fsa, .fna, .fa",
            // Any plain FASTQ file.
            AcceptFileTypes::Fastq => ".fastq, .fq",
        }
    }
}
