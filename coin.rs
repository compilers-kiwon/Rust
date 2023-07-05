fn main() {
    let num_of_500 = 10;
    let num_of_100 = 3;
    let num_of_50 = 10;
    let target = 3950;

    for _500 in 0..num_of_500+1 {
        for _100 in 0..num_of_100+1 {
            for _50 in 0..num_of_50+1 {
                let cur = 500*_500+100*_100+50*_50;

                if cur == target {
                    println!("500원x{}+100원x{}+50원x{}={}",_500,_100,_50,target);
                }
            }
        }
    }
}