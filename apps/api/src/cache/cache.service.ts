import { Inject, Injectable } from "@nestjs/common";
import { Cacheable } from "cacheable";

@Injectable()
export class CacheService {
  constructor(@Inject("CACHE_INSTANCE") private readonly cache: Cacheable) {}

  /**
   * Gets the value of the key. If the key does not exist in the primary store then it will check the secondary store.
   * @param {string} key The key to get the value of
   * @returns {Promise<T | undefined>} The value of the key or undefined if the key does not exist
   */
  async get<T>(key: string): Promise<T | undefined> {
    return this.cache.get<T>(key);
  }

  /**
   * Sets the value of the key. If the secondary store is set then it will also set the value in the secondary store.
   * @param {string} key the key to set the value of
   * @param {T} value The value to set
   * @param {number | string} [ttl] set a number it is miliseconds, set a string it is a human-readable
   * format such as `1s` for 1 second or `1h` for 1 hour. Setting undefined means that it will use the default time-to-live.
   * @returns {Promise<T | undefined>} The value set or null if the value was not set
   */
  async set<T>(
    key: string,
    value: T,
    ttl?: number | string,
  ): Promise<T | undefined> {
    const isSet = await this.cache.set(key, value, ttl);
    if (!isSet) {
      return undefined;
    }
    return value;
  }

  /**
   * Deletes the key from the primary store. If the secondary store is set then it will also delete the key from the secondary store.
   * @param {string} key The key to delete
   * @returns {Promise<boolean>} Whether the key was deleted
   */
  async delete(key: string): Promise<boolean> {
    return this.cache.delete(key);
  }
}
