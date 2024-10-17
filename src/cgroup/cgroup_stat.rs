use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CGroupStat {
    pub nr_descendants: u64,
    pub nr_dying_descendants: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseCGroupStatError;

impl FromStr for CGroupStat {
    type Err = ParseCGroupStatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stat = Self {
            nr_descendants: 0,
            nr_dying_descendants: 0,
        };

        s.lines()
            .map(|l| l.trim().split(" ").collect())
            .for_each(|s: Vec<&str>| match &*s[0] {
                "nr_descendants" => stat.nr_descendants = u64::from_str(s[1]).unwrap(),
                "nr_dying_descendants" => stat.nr_dying_descendants = u64::from_str(s[1]).unwrap(),
                _ => (),
            });

        Ok(stat)
    }
}
