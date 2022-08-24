import axios from "axios";
import * as uuid from "uuid";

function getSessionId() {
  const SESSION_STORAGE_KEY = "__telemetry_session";
  if (localStorage.getItem(SESSION_STORAGE_KEY) === null) {
    localStorage.setItem(SESSION_STORAGE_KEY, uuid.v4());
  }
  return localStorage.getItem(SESSION_STORAGE_KEY)!;
}

export interface Log<T> {
  sessionId: string;
  timestamp: number;
  commitHash: string;
  payload: T;
}

export class Telemetry {
  private sessionId: string;
  constructor(private url: string, private commitHash: string) {
    this.sessionId = getSessionId();
  }

  log<T>(endpoint: string, payload: T) {
    let host = window.location.hostname;
    if (host == "localhost" && !this.url.includes("localhost")) {
      return;
    }

    let log: Log<T> = {
      sessionId: this.sessionId,
      commitHash: this.commitHash,
      timestamp: new Date().getTime(),
      payload,
    };

    let fullUrl = this.url + "/" + endpoint;
    console.debug(`Logging to ${fullUrl}:`, log);
    axios.post(fullUrl, log);
  }
}

declare var COMMIT_HASH: string;
declare var TELEMETRY_URL: string;

declare global {
  var telemetry: Telemetry;
}

if (typeof window !== "undefined") {
  window.telemetry = new Telemetry(TELEMETRY_URL, COMMIT_HASH);
}
