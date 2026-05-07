#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<DnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<RnaNucleotide>);

#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    G,
    C,
    T,
    A,
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    G,
    C,
    U,
    A,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .map(|(index, nucleotide)| DnaNucleotide::new(nucleotide).ok_or(index))
            .collect::<Result<Vec<_>, _>>()
            .map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.into_iter().map(DnaNucleotide::into_rna).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .map(|(index, nucleotide)| RnaNucleotide::new(nucleotide).ok_or(index))
            .collect::<Result<Vec<_>, _>>()
            .map(Rna)
    }
}

impl DnaNucleotide {
    fn new(nucleotide: char) -> Option<Self> {
        match nucleotide {
            'G' => Some(Self::G),
            'C' => Some(Self::C),
            'T' => Some(Self::T),
            'A' => Some(Self::A),
            _ => None,
        }
    }

    fn into_rna(self) -> RnaNucleotide {
        match self {
            Self::G => RnaNucleotide::C,
            Self::C => RnaNucleotide::G,
            Self::T => RnaNucleotide::A,
            Self::A => RnaNucleotide::U,
        }
    }
}

impl RnaNucleotide {
    fn new(nucleotide: char) -> Option<Self> {
        match nucleotide {
            'G' => Some(Self::G),
            'C' => Some(Self::C),
            'U' => Some(Self::U),
            'A' => Some(Self::A),
            _ => None,
        }
    }
}
