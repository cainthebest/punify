declare module 'punify' {
  /**
   * Encodes a given string to ASCII using IDNA processing. Normalizes
   * characters (e.g., converting upper-case to lower-case) and applies Punycode encoding as necessary.
   * This process may fail, indicating potential syntax violations or other issues.
   * @param input The domain name to encode.
   * @returns The ASCII representation of the domain name.
   * @throws Will throw an error if the process fails.
   */
  export function toAscii(input: string): string;

  /**
   * Encodes a given string to ASCII with strict IDNA processing. Applies stricter rules
   * than `toAscii`, potentially rejecting valid IDNs that do not conform to stricter criteria.
   * @param input The domain name to encode strictly.
   * @returns The ASCII-encoded string if successful.
   * @throws Will throw an error detailing the specific reason for failure (e.g., syntax violations).
   */
  export function toAsciiStrict(input: string): string;

  /**
   * Decodes a given ASCII string to Unicode using IDNA processing. Includes normalizing
   * characters and decoding Punycode as necessary. Always returns a string for the mapped domain,
   * but may also indicate syntax violations or other processing errors.
   * @param input The ASCII-encoded domain name to decode.
   * @returns The Unicode representation of the domain name.
   * @throws Will throw an error if decoding fails due to syntax violations or other errors.
   */
  export function toUnicode(input: string): string;

  /**
   * Directly encodes a given Unicode string into a Punycode string.
   * @param input The Unicode string to encode.
   * @returns The encoded Punycode string, or an error if the input cannot be encoded.
   */
  export function encode(input: string): string;

  /**
   * Directly decodes a given Punycode string into a Unicode string.
   * @param input The Punycode string to decode.
   * @returns The decoded Unicode string, or an error if the input cannot be decoded.
   */
  export function decode(input: string): string;
}
