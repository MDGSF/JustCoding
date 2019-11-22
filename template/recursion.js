function recursion(level, param1, param2, ...) {

  // recursion terminator
  if (level > MAX_LEVEL) {
    // print result
    return;
  }

  // process logic in current level
  process_data(leve, data...);

  // drill down
  recursion(level + 1, p1, p2, ...);

  // reverse the current level status if needed
  reverse_state(level);
}
