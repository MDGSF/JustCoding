class DisjointSetItem {
  constructor(value, keyCallback) {
    this.value = value;
    this.keyCallback = keyCallback;
    this.parent = null;
    this.children = {};
  }

  getKey() {
    if (this.keyCallback) {
      return this.keyCallback(this.value);
    }
    return this.value;
  }

  getRoot() {
    return this.isRoot() ? this : this.parent.getRoot();
  }

  isRoot() {
    return this.parent === null;
  }

  getRank() {
    if (this.getChildren().length === 0) {
      return 0;
    }

    let rank = 0;

    this.getChildren().forEach(child => {
      rank += 1;
      rank += child.getRank();
    });

    return rank;
  }

  getChildren() {
    return Object.values(this.children);
  }

  setParent(parentItem, forceSettingParentChild = true) {
    this.parent = parentItem;
    if (forceSettingParentChild) {
      parentItem.addChild(this);
    }
    return this;
  }

  addChild(childItem) {
    this.children[childItem.getKey()] = childItem;
    childItem.setParent(this, false);
    return this;
  }
}

module.exports = DisjointSetItem;
