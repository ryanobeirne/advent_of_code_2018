#[test]
fn answers() {
	use super::*;
    assert_eq!(day_3::part_1(), 117505);
    assert_eq!(day_3::part_2(), 1254);
}

#[test]
fn splitter() {
    let split_comma: Vec<&str> = "450,602:".split(',').collect();
    println!("{:?}", split_comma);
    let split: Vec<u32> = split_comma.iter()
        .map(|s|
            s.trim_right_matches(':')
                .parse()
                .unwrap()
        ).collect();

    for i in split {
        println!("{}", i);
    }
}

#[test]
fn overlapper() {
    use day_3::Claim;

    let claims = Claim::claim_collect(
        vec![
            "#0 @ 1,3: 4x4".to_string(),
            "#1 @ 3,1: 4x4".to_string(),
            "#2 @ 5,5: 2x2".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
            "#4 @ 1,1: 2x2".to_string(),
            "#5 @ 1,1: 2x3".to_string(),
        ]
    );

    assert!(  &claims[0].rect.overlaps(&claims[1].rect) );
    assert!(! &claims[0].rect.overlaps(&claims[2].rect) );
    assert!(! &claims[1].rect.overlaps(&claims[2].rect) );
    assert!(  &claims[2].rect.overlaps(&claims[2].rect) );
    assert!(  &claims[2].rect.overlaps(&claims[3].rect) );
    assert!(! &claims[4].rect.overlaps(&claims[2].rect) );
    assert!(! &claims[4].rect.overlaps(&claims[0].rect) );
    assert!(  &claims[5].rect.overlaps(&claims[0].rect) );
}