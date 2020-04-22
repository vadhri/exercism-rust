use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut hs: HashSet<[u32; 3]> = HashSet::new();

    for trip_0 in 1..sum {
        for trip_1 in 1..sum - trip_0 {
            let trip_2 = sum - trip_0 - trip_1;
            if trip_0.pow(2) + trip_1.pow(2) == trip_2.pow(2) {
                let mut v = [trip_0, trip_1, trip_2];
                v.sort();

                hs.insert(v);
            }
        }
    }

    hs
}
