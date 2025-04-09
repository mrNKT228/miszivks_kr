export function formatDate(date: Date): string {
  function pad(input: number): string {
    return input.toString().padStart(2, "0");
  }

  return `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;
}
