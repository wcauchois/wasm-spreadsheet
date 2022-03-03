export function clamp(n: number, low: number, high: number) {
  if (n < low) {
    return low;
  }
  if (n > high) {
    return high;
  }
  return n;
}
