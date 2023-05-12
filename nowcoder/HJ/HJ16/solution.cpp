/*
dp[i][j]: 前i个物品，背包重量为j的情况下能装的最大价值
dp[i][j] = max(
             dp[i-1][j],               第i个物品不放入背包
             dp[i-1][j-w[i]] + v[i],   第i个物品放入背包
           )

k =（主件，主件+附件1，主件+附件2，主件+附件1+附件2）
dp[i][j] = max(
            dp[i-1][j],                   第i个物品不放入背包
            dp[i-1][j-w[i][k]] + v[i][k], 第i个主件的第k种情况放入背包
           )
*/
#include <iostream>
#include <map>
#include <vector>

// 附件
struct Attachment {
  int id;
  int master_id;  // 主键编号
  int v;          //价格
  int p;          //重要度
};

// 主件
struct Master {
  int id;
  int v;  //价格
  int p;  //重要度
  std::vector<int> attachment_ids;
};

int main() {
  int n = 0;  // 总钱数
  int m = 0;  // 可购买物品数
  std::cin >> n >> m;

  std::map<int, Master> master_map;
  std::map<int, Attachment> attachment_map;
  for (int i = 1; i <= m; ++i) {
    int v = 0;  // 价格
    int p = 0;  // 重要度
    int q = 0;  // =0表示主件，>0表示附件
    std::cin >> v >> p >> q;

    if (q == 0) {
      // 主件
      Master master;
      master.id = i;
      master.v = v;
      master.p = p;
      master_map[i] = master;
    } else {
      // 附件
      Attachment attachment;
      attachment.id = i;
      attachment.master_id = q;
      attachment.v = v;
      attachment.p = p;
      attachment_map[i] = attachment;
    }
  }

  for (auto& item : attachment_map) {
    master_map[item.second.master_id].attachment_ids.push_back(item.first);
  }

  std::vector<std::vector<int>> dp;
  dp.resize(m, std::vector<int>(6, 0));

  return 0;
}
