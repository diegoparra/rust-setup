# Labs

### Create a File Reader Application

- Ask the user the file path and save that to a input variable
    - HINT: use std::io::sdtin ....
- create a variable 'file' and open the file using File package from the user input
- run a match against file
- check by OK()
    - open a reader using BufReader::new()
    - loop over all lines
        HINT: reader.lines()
    - match over line
        - check OK() 
            - print the line content
        - check Err()
            - panic reading the file content
- check Err() 
    - panic opening the file
