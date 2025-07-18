# Read_int
A simple Rust crate for getting user input

## About
### This crate is made for helping in getting user inputs in other types than String!
#### Supported types:
- unsigned
- signed
- float
- bool

## Functions:

### Numbers: 
<img width="775" height="30" alt="image" src="https://github.com/user-attachments/assets/0d3e4907-5921-4285-90cf-aecba0173c3d" />

This function receives a generic type (all numbers type) and return a Option with the same type of the entry. <br>
You can also gives the function a text to display to user.

### Bool:
<img width="565" height="27" alt="image" src="https://github.com/user-attachments/assets/acd51a73-103e-42cf-9db0-e5c3e6c1cfc4" />

This function return a Option of a bool value (true or false). <br>
Like the other function, you can also gives a text to display to user. <br>
To receive **TRUE** , the user need to write (**yes** or **YES**), any other words will return **FALSE**.
