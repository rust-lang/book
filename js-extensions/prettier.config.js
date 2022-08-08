module.exports = {
  printWidth: 100,
  tabWidth: 2,
  arrowParens: "avoid",
  importOrder: ["<THIRD_PARTY_MODULES>", "^[./]"],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
  parser: "typescript",
  plugins: [require("@trivago/prettier-plugin-sort-imports")],
};
