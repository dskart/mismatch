// Types
interface HighScore {
  score: number;
  word1: string;
  word2: string;
}

interface HTMLInputWithStyle extends HTMLInputElement {
  style: CSSStyleDeclaration;
}

// Constants
const LOCAL_STORAGE_KEY = "highScore";
const WORD_MAX_LENGTH = 20;
const WORD_REGEX = /^[A-Za-z]*$/;
const SHARE_URL = "https://mismatch.raphaelvanhoffelen.com";

// Helper functions
function getElement<T extends HTMLElement | SVGElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Element with id '${id}' not found`);
  return element as T;
}

// Main functions
export function getHighScore(): HighScore | null {
  const storedScore = localStorage.getItem(LOCAL_STORAGE_KEY);
  return storedScore ? JSON.parse(storedScore) : null;
}

export function updateHighScore(
  newScore: number,
  word1: string,
  word2: string
): void {
  const currentHigh = getHighScore();
  const newHighScore: HighScore = {
    score: newScore,
    word1,
    word2,
  };

  if (currentHigh && currentHigh.score > newScore) {
    return; // Don't update if current high score is better
  }

  localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(newHighScore));
  displayHighScore();
}

export function displayHighScore(): void {
  const highScore = getHighScore();
  const highScoreDisplay = getElement<HTMLElement>("highScoreDisplay");

  if (!highScore) {
    highScoreDisplay.style.display = "none";
    return;
  }

  try {
    const highScoreText = getElement<HTMLElement>("highScoreText");
    const highScoreWords = getElement<HTMLElement>("highScoreWords");

    highScoreText.textContent = `High Score: ${highScore.score}`;
    highScoreWords.textContent = `Words: ${highScore.word1}, ${highScore.word2}`;
    highScoreDisplay.style.display = "block";
  } catch (error) {
    console.error("Error displaying high score:", error);
  }
}

export function validateInput(input: HTMLInputWithStyle): void {
  const isValid =
    WORD_REGEX.test(input.value) && input.value.length <= WORD_MAX_LENGTH;

  // Update input styling
  input.style.borderColor = isValid ? "" : "red";
  input.style.color = isValid ? "" : "red";

  // Validate submit button
  try {
    const word1Input = document.querySelector<HTMLInputElement>(
      'input[name="word1"]'
    );
    const word2Input = document.querySelector<HTMLInputElement>(
      'input[name="word2"]'
    );
    const submitBtn = getElement<HTMLButtonElement>("submitBtn");

    if (!word1Input || !word2Input) {
      throw new Error("Word input elements not found");
    }

    const isWord1Valid = WORD_REGEX.test(word1Input.value);
    const isWord2Valid = WORD_REGEX.test(word2Input.value);

    submitBtn.disabled = !(
      word1Input.value &&
      word2Input.value &&
      isWord1Valid &&
      isWord2Valid
    );
  } catch (error) {
    console.error("Error validating input:", error);
  }
}

export async function copyHighScore(): Promise<void> {
  const highScore = getHighScore();
  if (!highScore) return;

  const shareText = [
    `🎯Checkout Out My High Score: ${highScore.score}`,
    `📝 Words: ${highScore.word1} | ${highScore.word2}`,
    `\nPlay at: ${SHARE_URL}`,
  ].join("\n");

  try {
    await navigator.clipboard.writeText(shareText);

    const copyIcon = document.getElementById("copyIcon");
    const copyText = getElement<HTMLElement>("copyText");

    if (!copyIcon) {
      throw new Error("Copy icon element not found");
    }

    // Update UI to show copied state
    copyIcon.innerHTML =
      '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />';
    copyText.textContent = "Copied!";

    // Reset UI after delay
    setTimeout(function () {
      copyText.textContent = "Copy High Score!";
      copyIcon.innerHTML =
        '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />';
    }, 2000);
  } catch (error) {
    console.error("Error copying high score:", error);
  }
}

// Expose functions to window
declare global {
  interface Window {
    copyHighScore: () => Promise<void>;
    validateInput: (input: HTMLInputWithStyle) => void;
  }
}

(function () {
  window.copyHighScore = copyHighScore;
  window.validateInput = validateInput;
})();
