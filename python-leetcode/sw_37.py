# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:
  def serialize(self, root):
    """Encodes a tree to a single string.

    :type root: TreeNode
    :rtype: str
    """
    result = []
    def dfs(root):
      if not root:
        result.append("null")
        return
      result.append(str(root.val))
      dfs(root.left)
      dfs(root.right)
    dfs(root)
    return ','.join(result)


  def deserialize(self, data):
    """Decodes your encoded data to tree.

    :type data: str
    :rtype: TreeNode
    """
    array = data.split(",")
    def dfs(array):
      if len(array) == 0: return None
      first = array.pop(0)
      if first == "null": return None
      node = TreeNode(int(first))
      node.left = dfs(array)
      node.right = dfs(array)
      return node
    return dfs(array)


# Your Codec object will be instantiated and called as such:
# codec = Codec()
# codec.deserialize(codec.serialize(root))

