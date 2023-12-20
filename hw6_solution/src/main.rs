fn fizzbuzz(){
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    for i in 0..301{
        if i%3 == 0{
            count1 +=1;
            println!("fizz");}
        if i%5 == 0{
            count2 +=1;
            println!("buzz");}
        if i%15 == 0 {
            count3 +=1;
            println!("fizz buzz");}

    }
    println!("number of fizz occurances{}, number of buzz occurances{},number of fizzbuzz occurances{}",count1,count2,count3,);
}
fn main() {
    println!("Hearty Welcome!");
    fizzbuzz();

}
