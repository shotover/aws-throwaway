#[non_exhaustive]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
pub enum PlacementStrategy {
    Cluster,
    Partition,
    Spread,
    Unknown,
}
impl ::std::convert::From<&str> for PlacementStrategy {
    fn from(s: &str) -> Self {
        match s {
            "cluster" => PlacementStrategy::Cluster,
            "partition" => PlacementStrategy::Partition,
            "spread" => PlacementStrategy::Spread,
            _other => PlacementStrategy::Unknown,
        }
    }
}
impl ::std::str::FromStr for PlacementStrategy {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(PlacementStrategy::from(s))
    }
}
impl PlacementStrategy {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            PlacementStrategy::Cluster => "cluster",
            PlacementStrategy::Partition => "partition",
            PlacementStrategy::Spread => "spread",
            PlacementStrategy::Unknown => "unknown",
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["cluster", "partition", "spread"]
    }
}
impl ::std::convert::AsRef<str> for PlacementStrategy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
