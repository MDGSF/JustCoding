const { List } = require("../list/List.js");

const defaultHashTableSize = 32;

class HashTable {
  constructor(hashTableSize = defaultHashTableSize) {
    this.buckets = Array(hashTableSize)
      .fill(null)
      .map(() => new List());

    this.keys = {};
  }

  /*
  @brief: Converts key string to hash number.
  @param: {string} key
  @return: {number}
  */
  hash(key) {
    const hash = Array.from(key).reduce(
      (hashAccumulator, keySymbol) => hashAccumulator + keySymbol.charCodeAt(0),
      0
    );

    return hash % this.buckets.length;
  }

  /*
  @brief: find key in bucket's list.
  @return: {Element}
  */
  find(list, key) {
    for (let e = list.Front(); e != null; e = e.Next()) {
      if (e.value.key === key) {
        return e;
      }
    }
    return null;
  }

  /*
  @brief: set key=value to hash table.
  */
  set(key, value) {
    const keyhash = this.hash(key);
    this.keys[key] = keyhash;
    const bucketList = this.buckets[keyhash];
    const elem = this.find(bucketList, key);

    if (!elem) {
      bucketList.PushBack({ key, value });
    } else {
      elem.value.value = value;
    }
  }

  delete(key) {
    const keyhash = this.hash(key);
    delete this.keys[key];
    const bucketList = this.buckets[keyhash];
    const elem = this.find(bucketList, key);

    if (elem) {
      bucketList.Remove(elem);
    }

    return null;
  }

  get(key) {
    const keyhash = this.hash(key);
    const bucketList = this.buckets[keyhash];
    const elem = this.find(bucketList, key);

    if (elem) {
      return elem.value.value;
    }
    return null;
  }

  has(key) {
    return Object.hasOwnProperty.call(this.keys, key);
  }

  getkeys() {
    return Object.keys(this.keys);
  }
}

module.exports = HashTable;
