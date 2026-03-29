type Position = readonly [number, number]

function validPosition(position: Position) {
  if (position[0] < 0 || position[0] > 7 || position[1] < 0 || position[1] > 7) {
    throw new Error("Queen must be placed on the board");
  }
}

function equalsPosition(a: Position, b: Position): boolean {
  return a[0] === b[0] && a[1] === b[1];
}

type Positions = {
  white: Position
  black: Position
}
export class QueenAttack {
  public readonly black: Position
  public readonly white: Position

  // white: [whiteRow, whiteColumn]
  // black: [blackRow, blackColumn]
  constructor(queens: Partial<Positions> = {}) {
    this.black = queens.black || [0, 3];
    validPosition(this.black);
    this.white = queens.white || [7, 3];
    validPosition(this.white);
    if (equalsPosition(this.black, this.white)) {
      throw new Error("Queens cannot share the same space");
    }
  }

  toString() {
    let res: string = '';
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        const position: Position = [i, j];
        if (equalsPosition(position, this.black)) {
          res += 'B';
        } else if (equalsPosition(position, this.white)) {
          console.log(position);
          res += 'W';
        } else {
          res += '_';
        }
        if (j < 7) {
          res += ' ';
        }
      }
      if (i < 7) {
        res += '\n';
      }
    }
    return res;
  }

  get canAttack() {
    if (this.black[0] == this.white[0] || this.black[1] == this.white[1]) {
      return true;
    }

    for (let i = 1; i < 8; i++) {
      const positions:Position[] = [
        [this.black[0] - i, this.black[1] - i], [this.black[0] + i, this.black[1] - i],
        [this.black[0] - i, this.black[1] + i], [this.black[0] + i, this.black[1] + i]
      ]
      for (let idx in positions) {
        if (equalsPosition(positions[idx], this.white)) {
          return true;
        }
      }
    }
    return false;
  }

}
