pub fn sonar_sweep(sea_floor: &[i32]) -> i32 {
    sea_floor.windows(2).filter(|win| win[1] > win[0]).count() as i32
}

pub fn triple_sonar(sea_floor: &[i32]) -> i32 {
    sea_floor
        .windows(3)
        .zip(sea_floor.windows(3).skip(1))
        .filter(|(left, right)| 
            right.iter().sum::<i32>() > left.iter().sum::<i32>()
        )
        .count() as i32
        
}
