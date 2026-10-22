import { SendTransactionError } from "@solana/web3.js";
import { AnchorError, translateError } from "../src/error";
import { sendTransactionError } from "../src/utils/rpc";

// Logs of a real failed `increment` of the historical demo program.
const logs = [
  "Program 59eu89dUdFWrGzCzT6WzjpPmn9VnwmG4a3NwFmxHu8MN invoke [1]",
  "Program log: Instruction: Increment",
  "Program log: AnchorError thrown in programs/demo/src/instructions/increment.rs:13. Error Code: Unauthorized. Error Number: 6000. Error Message: Only the counter authority can update this counter.",
  "Program 59eu89dUdFWrGzCzT6WzjpPmn9VnwmG4a3NwFmxHu8MN consumed 3343 of 200000 compute units",
  "Program 59eu89dUdFWrGzCzT6WzjpPmn9VnwmG4a3NwFmxHu8MN failed: custom program error: 0x1770",
];

describe("sendTransactionError", () => {
  for (const action of ["send", "simulate"] as const) {
    it(`keeps program logs with the installed @solana/web3.js constructor (${action})`, () => {
      const error = sendTransactionError(
        action,
        "Raw transaction failed",
        logs,
        "signature"
      );
      expect(error).toBeInstanceOf(SendTransactionError);
      expect(error.logs).toEqual(logs);
      expect(error.message).not.toContain("Unknown action");
      const translated = translateError(error, new Map());
      expect(translated).toBeInstanceOf(AnchorError);
      expect((translated as AnchorError).error.errorCode).toEqual({
        code: "Unauthorized",
        number: 6000,
      });
    });
  }
});
