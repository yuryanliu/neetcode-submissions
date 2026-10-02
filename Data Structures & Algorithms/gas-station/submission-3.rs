impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_tank = 0;
        let mut curr_tank = 0;
        let mut starting_station = 0;

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            total_tank += diff;
            curr_tank += diff;
            if curr_tank < 0 {
                starting_station = (i + 1) as i32;
                curr_tank = 0;
            }
        }

        if total_tank >= 0 {
            starting_station
        } else {
            -1
        }
    }
}