const object = {
  "\uD8001": async () => 42,
  "\uDC001": async () => 42,
  "!1": async () => 42,
  "!2": async function (a, b) {
    return a + b;
  },
};
