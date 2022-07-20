import { Board, Agent } from "web-tac-toe";

const ttt = Board.new();
const agent = Agent.new();

console.log(ttt.get_available_squares());

const squares = [
  document.getElementById("square0"),
  document.getElementById("square1"),
  document.getElementById("square2"),
  document.getElementById("square3"),
  document.getElementById("square4"),
  document.getElementById("square5"),
  document.getElementById("square6"),
  document.getElementById("square7"),
  document.getElementById("square8"),
];

function handleCellPlayed(clickedCell, clickedCellIndex) {
  ttt.claim_square(clickedCellIndex);
  clickedCell.innerHTML = ttt.get_symbol(clickedCellIndex);
}

function handleCellClick(clickedCellEvent) {
  if (ttt.is_game_over()) {
    outcome();
    return;
  }

  const clickedCell = clickedCellEvent.target;
  const clickedCellIndex = parseInt(clickedCell.getAttribute("index"));

  //setColor(clickedCell, 100, 200, 50);
  handleCellPlayed(clickedCell, clickedCellIndex);

  console.log(ttt.check_for_win());

  //board_shader();

  if (ttt.is_game_over()) {
    outcome();
    return;
  }

  // let sqs = ttt.get_available_squares();
  // const choice = agent.get_choice(sqs);
  // console.log(choice);

  // ttt.claim_square(choice);
}

function board_shader() {
  let sq = ttt.square_weight(4);
  setColor(squares[4], 0, 255, 0, 0.2);
}

function reset() {
  ttt.reset();
  for (let i = 0; i < squares.length; i++) {
    squares[i].innerText = ttt.get_symbol(i);
    squares[i].style.backgroundColor = "";
  }
  const outcome = document.getElementById("outcome");
  outcome.innerText = "";
}

function outcome() {
  const outcome = document.getElementById("outcome");
  outcome.innerText = ttt.check_for_win() + " is the winner!";
}

function setColor(element, red, green, blue, alpha) {
  element.style.backgroundColor = "rgba(" + red + "," + green + "," + blue + "," + alpha + ")";
}

document.querySelectorAll(".square").forEach((cell) => cell.addEventListener("click", handleCellClick));
document.getElementById("reset").addEventListener("click", reset);
