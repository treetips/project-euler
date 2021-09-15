module.exports = {
  roots: ["<rootDir>/src"],
  testMatch: [
    "**/__tests__/**/*.(ts|tsx|js)",
    "**/?(*.)+(spec|test).(ts|tsx|js)",
  ],
  testPathIgnorePatterns: ["<rootDir>/node_modules/", "<rootDir>/build/"],
  transformIgnorePatterns: ["<rootDir>/node_modules/", "<rootDir>/build/"],
  transform: {
    "^.+\\.(ts|tsx)$": [
      "@swc/jest",
      {
        module: {
          type: "commonjs",
        },
        jsc: {
          parser: {
            syntax: "typescript",
            tsx: false,
          },
        },
      },
    ],
  },
};
