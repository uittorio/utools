# Usudoku

Just having fun implementing the sudoku game

This project is to learn the maths around sudoku, especially learning algorithms to generate a valid sudoku board with one solution

Phase 1
Implement a ~playbable sudoku

V Do not let changing existing pre filled cells
V Show error
V Annotations
V Win state 
 - block board
 - show something

Phase 2 (current)
Ensure the sudoku has one solution by improving how the clues are generated;
Add also difficulty selection

Phase 3
Learn and use the dancing algorithm to prefill the sudoku instead of backtracking

Phase 4
Show clues to the user, like naked single etc

Refactor
- hardcoded numbers (grid concept is a bit of everywhere). Do I want to make this board compatibile with other variants ? 
- backtracking is not a generation strategy but it is a finding strategy. We will make this better once we implement a different algorithm to generate the sudoku board

Extras
Undo/Redo
