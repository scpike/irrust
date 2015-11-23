use std::collections::HashSet;

use normalize::normalize;

fn shingles(s: &str) -> HashSet<String> {
    let chars: Vec<_> = s.chars().collect();
    chars.windows(2).map(|w| w.iter().cloned().collect()).collect()
}

// Intersection of the sets divided by the size of the union of the
// sets.
fn jaccard_distance(s1: &str, s2: &str) -> f64 {
    let s1_shingles = shingles(s1);
    let s2_shingles = shingles(s2);
    let inter = s1_shingles.intersection(&s2_shingles).count();
    let union = s1_shingles.union(&s2_shingles).count();
    (inter as f64) / (union as f64)
}

fn numeric_match(s1: &str, s2: &str) -> bool {
    s1.chars().filter(|l| l.is_numeric()).eq(s2.chars().filter(|l| l.is_numeric()))
}

pub fn compare_normals(s1: &str, s2: &str) -> f64 {
    if numeric_match(s1, s2) {
        jaccard_distance(s1, s2)
    } else {
        0.0
    }
}

pub fn compare(s1: &str, s2: &str) -> f64 {
    let normal_s1 = normalize(s1);
    let normal_s2 = normalize(s2);
    compare_normals(&normal_s1, &normal_s2)
}

// fn comp_and_print(s1: &str, s2: &str) {
//     println!("'{}' vs '{}' ... \t {}", s1, s2, compare(s1, s2));
// }

#[cfg(test)]

mod tests {
    use super::compare;
    use super::numeric_match;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(compare("Pear", "Peach"), 0.4);
    }

    #[test]
    fn compares_numbers() {
        assert_eq!(numeric_match("Pear 123", "Peach 123"), true);
        assert_eq!(numeric_match("Pear 121", "Peach 123"), false);
        assert_eq!(numeric_match("1800 Tequilla", "1800 Tequilla"), true);
        assert_eq!(numeric_match("Import Z1", "Import Z2"), false);
    }

    #[bench]
    fn bench_compare_one_word(b: &mut Bencher) {
        b.iter(|| compare("Pear", "Peach"));
    }

    #[bench]
    fn bench_compare_longer(b: &mut Bencher) {
        b.iter(|| compare("Apple pie", "Grandma's Apple pie"));
    }

    #[bench]
    fn bench_compare_longest(b: &mut Bencher) {
        b.iter(|| compare("American Wine Dist.- We Miolo", "American Wine Distributors"));
    }
}
