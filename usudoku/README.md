# Usudoku

Just having fun implementing sudoku

This project is to learn the maths around sudoku, especially learning algorithms to
- generating a valid sudoku board with one solution
- fill the sudoku board with efficient algoroithm


Phase 1 (current)
Implement a ~playbable sudoku

- Do not let changing existing pre filled cells
- Show error

Phase 2
Ensure the sudoku has one solution by improving how the clues are generated

Phase 3
Learn and use the dancing algorithm to prefill the sudoku

Phase 4
Show clues to the user, like naked single etc

Refactor
- hardcoded numbers (grid concept is a bit of everywhere). Do I want to make this board compatibile with other variants ? 
- backtracking is not a generation strategy but it is a finding strategy. We will make this better once we implement a different algorithm to generate the sudoku board
