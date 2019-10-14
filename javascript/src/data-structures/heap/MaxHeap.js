const Heap = require("./Heap");

class MaxHeap extends Heap {
  pairIsInCorrentOrder(firstElement, secondElement) {
    return this.compare.greaterThanOrEqual(firstElement, secondElement);
  }
}

module.exports = MaxHeap;
