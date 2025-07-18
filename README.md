# Read_int
A simple Rust crate for getting user input

# About
**This crate was made for helping in getting user inputs in others types!**  
**Supported types:**  
- **generic_read:**
   - unsigned
   - signed
   - float
- **vec_read:**
  - unsigned
  - signed
  - float
  - bool
  - char
  - String

# Functions:

### Numbers: 
<img width="775" height="30" alt="image" src="https://github.com/user-attachments/assets/0d3e4907-5921-4285-90cf-aecba0173c3d" />

- This function receives a generic type **(T)** (all numbers type), and return a Option with the same type.  
- You can also gives the function a text to display to user.

### Bool:
<img width="565" height="27" alt="image" src="https://github.com/user-attachments/assets/acd51a73-103e-42cf-9db0-e5c3e6c1cfc4" />

- This function return a bool value (true or false).  
- Like the other function, you can also gives a text to display to user.  
- To receive **TRUE** , the user need to write (**yes** or **YES**), any other words will return **FALSE**.

### Vector:
<img width="791" height="29" alt="image" src="https://github.com/user-attachments/assets/41169b22-ef50-421a-b781-1da1b3a73d32" />

- This function receives a generic type **(T)** for a vector, and return a Option of a vector with the same type.  
- Like any other function in Read_int, you can pass a text to display to user.  
- This function accept all types included in the **SUPPORTED TYPES**, and the input need a space between each other!  
- **E.g:** <img width="227" height="23" alt="image" src="https://github.com/user-attachments/assets/46b4ab8e-8d76-4ec3-bf7f-ec0b46cee6a5" />
