//patika_bootcamp_customfilter 10.08.2023
use std::vec;

// first step is creating a struct of int. named FilterCondition
struct FilterCondition{
    filter: i32
}
//imlementation of FilterCond.
impl FilterCondition {
    //comparing struct (self as an reference) with an item and returning bool as a result
    fn is_match(&self,item :i32) -> bool{
        item%self.filter==0//filtering even numbers within the list
    }

}
fn custom_filter(list: Vec<i32>,condition: &FilterCondition) -> Vec <i32>{
    return list.into_iter()
    .filter(|item:&i32| condition
    .is_match(*item)).collect(); //using iteration to conrol after filter list changes so used .collect
}
fn main(){
    //creating a list of counting numbers to test
    let testlist:Vec<i32>=vec![1,2,3,4,5,6,7,8,9];
    //creating a condition 
    let filter_condition: FilterCondition=FilterCondition { filter: (2) };
    
    let final_list:Vec<i32>=custom_filter(testlist, &filter_condition);
    println!("Even cardinal numbers:{:#?}",final_list);
}