/**
 * Example TypeScript file for testing code agent execution
 *
 * This file provides simple functions that can be modified, tested,
 * and extended during instruction execution.
 *
 * Part of architect-agent skill v4.1
 */

/**
 * Adds two numbers together
 * @param a First number
 * @param b Second number
 * @returns Sum of a and b
 */
export function add(a: number, b: number): number {
  return a + b;
}

/**
 * Multiplies two numbers
 * @param a First number
 * @param b Second number
 * @returns Product of a and b
 */
export function multiply(a: number, b: number): number {
  return a * b;
}

/**
 * Greets a person by name
 * @param name Person's name
 * @returns Greeting message
 */
export function greet(name: string): string {
  return `Hello, ${name}!`;
}

/**
 * Interface for a User
 */
export interface User {
  id: number;
  name: string;
  email: string;
  createdAt: Date;
}

/**
 * Creates a new user object
 * @param id User ID
 * @param name User name
 * @param email User email
 * @returns User object
 */
export function createUser(id: number, name: string, email: string): User {
  return {
    id,
    name,
    email,
    createdAt: new Date(),
  };
}

/**
 * Validates an email address (simple validation)
 * @param email Email address to validate
 * @returns True if valid, false otherwise
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

/**
 * Main function for testing
 */
export function main(): void {
  console.log("Example TypeScript file loaded");
  console.log("Add 2 + 3 =", add(2, 3));
  console.log("Multiply 4 * 5 =", multiply(4, 5));
  console.log(greet("Code Agent"));

  const user = createUser(1, "Test User", "test@example.com");
  console.log("Created user:", user);
  console.log("Email valid?", isValidEmail(user.email));
}

// Run main if this file is executed directly
if (require.main === module) {
  main();
}
