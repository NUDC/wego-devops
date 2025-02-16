import type { App } from 'vue';
import { EventEmitter } from 'events';
import dayjs from 'dayjs';

type Listener = (...args: any[]) => void;

interface SocketMessage {
  eventName: string;
  args: any;
}

interface DevOpsWebSocketOptions {
  url: string | URL;
  heartbeatInterval?: number;
  heartbeatTimeout?: number;
  reconnectionInterval?: number;
  maxListeners?: number;
}
class WebSocketEvent {
  #options: DevOpsWebSocketOptions = {
    url: 'ws://localhost:8080',
    heartbeatInterval: 5,
    heartbeatTimeout: 5,
    reconnectionInterval: 5,
    maxListeners: 50,
  };
  #ws!: WebSocket;
  #emit!: EventEmitter;
  constructor(options: DevOpsWebSocketOptions) {
    this.#emit = new EventEmitter();
    this.#options = Object.assign(this.#options, options);
    this.#connection();
    this.#Listener();
  }
  on(eventName: string | symbol, listener: Listener) {
    this.#emit.on(eventName, listener);
  }
  once(eventName: string | symbol, listener: Listener) {
    this.#emit.once(eventName, listener);
  }
  off(eventName: string | symbol) {
    this.#emit.removeAllListeners(eventName);
  }
  isExist(eventName: string | number) {
    return this.#emit.eventNames().findIndex(o => o == eventName) > -1;
  }
  emit(type: string, args: string) {
    if (this.#ws.readyState != 1) {
      console.log(`send error:websocket readystate=${this.#ws.readyState}`);
      return;
    }
    const msg: SocketMessage = {
      eventName: type,
      args: args,
    };
    this.#ws.send(JSON.stringify(msg));
  }
  #maxListener() {
    this.#emit.setMaxListeners(this.#options.maxListeners ?? 50);
  }
  #heartbeat() {
    let startTime = dayjs();
    setInterval(() => {
      startTime = dayjs();
      this.emit('hearbeat', 'ping');
    }, 1000 * (this.#options.heartbeatInterval ?? 5));

    this.on('hearbeat', () => {
      const seconds = dayjs().diff(startTime, 'seconds');
      if ((this.#options.heartbeatTimeout ?? 5) < seconds) {
        console.log('heartbeat timeout');
        this.#connection();
      }
    });
  }
  #connection() {
    this.#ws = new WebSocket(this.#options.url);
  }
  #reconnection() {
    // 1 ：对应常量OPEN(numeric value 1)，
    // 连接成功建⽴，可以进⾏通信。The WebSocket connection is established and communication is possible.
    // 2 ：对应常量CLOSING(numeric value 2)
    // 连接正在进⾏关闭握⼿，即将关闭。The connection is going through the closing handshake.
    // 3 : 对应常量CLOSED(numeric value 3)
    // 连接已经关闭或者根本没有建⽴。The connection has been closed or could not be opened.
    setInterval(() => {
      if (this.#ws.readyState != 1) {
        console.log('reconnection');
        this.#connection();
      }
    }, 1000 * (this.#options.reconnectionInterval ?? 5));
  }
  #Listener() {
    this.#maxListener();
    this.#reconnection();
    this.#heartbeat();
    this.#ws.onclose = ev => {
      if (this.isExist('close')) {
        this.#emit.emit('close', ev);
      }
    };
    this.#ws.onerror = ev => {
      if (this.isExist('error')) {
        this.#emit.emit('error', ev);
      }
    };
    this.#ws.onopen = ev => {
      if (this.isExist('open')) {
        this.#emit.emit('open', ev);
      }
    };
    this.#ws.onmessage = (ev: MessageEvent) => {
      const message = JSON.parse(ev.data) as SocketMessage;
      if (this.isExist(message.eventName)) {
        this.#emit.emit(message.eventName, message.args);
      }
    };
  }
}

export default function VueSocket(app: App): void {
  if (app.config.globalProperties.$webSocketEvent != null) {
    return;
  }
  const webSocketEvent = new WebSocketEvent({
    url: `${import.meta.env.VITE_PROXY_WS_API}?id=${new Date().getTime()}`,
    heartbeatInterval: 5,
    reconnectionInterval: 5,
  });
  app.config.globalProperties.$webSocketEvent = webSocketEvent;
}

export function GetWebSocketEvent(): WebSocketEvent {
  return getCurrentInstance()?.appContext.config.globalProperties.$webSocketEvent;
}
