const object = {
  "\uD8001": async () => 42,
  "\uDC001": async () => 42,
  "!1": async () => 42,
  "!2": async function (a, b) {
    return a + b;
  },
};

return (async () => {
  expect(await object["\uD8001"]()).toBe(42);
  expect(await object["\uDC001"]()).toBe(42);
  expect(await object["!1"]()).toBe(42);
  expect(await object["!2"](1, 2)).toBe(3);
})();
