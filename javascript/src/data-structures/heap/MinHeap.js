const Heap = require("./Heap");

class MinHeap extends Heap {
  pairIsInCorrentOrder(firstElement, secondElement) {
    return this.compare.lessThanOrEqual(firstElement, secondElement);
  }
}

module.exports = MinHeap;
