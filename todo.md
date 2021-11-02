- Lrinser Project

    - Lexer

        - Iterate through characters identifiying patterns to locate 'pieces' (ie. tokens).
         
         
    - Parser
         
        - Loop through tokens identifiying patterns to create an abstract syntax tree (used for identifying order of operations).
         
         
    - Simplifier
         
        - Take the ast and simplify them:
             
            - Move all non `y` variables to the right side of the equation, and all `y` variables to the left.
             
            - Move all non `y` variables from the left side of the equation to the right.
             
             
    - Interpreter
         
        - Take headers and generate image frame.
             
        - Identify valid equations
             
             
    - Renderer
         
        - Draw valid equations into an image or render it as data.
             
             
    - Printer
         
        - Prepare for laser printing.
             
        - Send to laser printer.



- Resources:
     
    - [math.wpi.edu](https://www.math.wpi.edu/IQP/BVCalcHist/calc5.html)