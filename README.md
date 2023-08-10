# patika_bootcamp_customfilter<br />
## TASK: Building a Custom Filtering Function in Rust <br />
In this task, you will create a custom filtering function in Rust that allows filtering elements from a given collection based on a specific condition. The goal is to implement a beginner-friendly solution that avoids using closures to simplify the understanding of the code.<br />
## Steps<br />
+ Create a new Rust project by running the following command in the terminal:<br />
cargo new my_project <br />
+ Open the main.rs file in a text editor.<br />
+ Define a struct called FilterCondition with a single field of the desired type for filtering.<br />
+ Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.<br />
+ Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.<br />
+ In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.<br />
+ Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.<br />
+ Print the filtered result to the console.<br />
+ Compile and run the program to test its functionality.<br />
## Checklist<br />
+ Create a new Rust project and open the main.rs file.<br />
+ Define the FilterCondition struct with the desired type for filtering.<br />
+ Implement the is_match method on the FilterCondition struct.<br />
+ Define the custom_filter function to filter elements based on the condition.<br />
+ Create a collection and a FilterCondition object in the main function.<br />
+ Call the custom_filter function and store the result.<br />
+ Print the filtered result to the console.<br />
+ Compile and run the program to test its functionality.<br />
