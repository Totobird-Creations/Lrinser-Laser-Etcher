- [ ] Lrinser Project:

    - [x] Lexer

        - [x] Iterate through characters identifiying patterns to locate 'pieces' (ie. tokens).
         
         
    - [x] Parser
         
        - [ ] Loop through tokens identifiying patterns to create an abstract syntax tree (used for identifying order of operations).
         
         
    - [ ] Simplifier
         
        - [ ] Take the ast and simplify them:
             
            - [ ] Move all non `y` variables to the right side of the equation, and all `y` variables to the left.
             
            - [ ] Move all non `y` variables from the left side of the equation to the right.
             
             
    - [ ] Interpreter
         
        - [x] Take headers and generate image frame.
             
        - [x] Identify valid equations
             
             
    - [ ] Renderer
         
        - [x] Draw valid equations into an image or render it as data.
             
             
    - [ ] Printer
        - [x] Send to printer.

        - [ ] Prepare for laser printing.
             
        - [ ] Send to laser printer.



- Bugs:

    - Circular, Conic, etc shapes have holes in them when the line is almost vertical.



- Upcoming:

    - Print on Linux.

    - Send to laser cutter.

    - Unary operations

    - Power

    - Optimise renderer (Remove evaluating equations multiple times.)



- Resources:
     
    - [math.wpi.edu](https://www.math.wpi.edu/IQP/BVCalcHist/calc5.html)

    - [github.com](https://github.com/m4dh0rs3/cas/)
