/** Nodejs wrapper for Hycon difficulty adjustment written in Rust */
export as namespace DifficultyAdjuster;
/** Adjsuts the difficulty target for the next block based on the EMA parameters provided */
export function adjust(workEMA: number, timeEMA: number): number;
/** Calculates a new EMA value */
export function ema(newValue: number, previousValue: number, alpha: number): number;


