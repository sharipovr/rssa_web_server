Very high-level plan/memo.

1) Create a workspace with four multi-projects.
2) Start the development with basic tcp server. Run it with: ``` cargo run -p tcpserver```
3) Add even more primitive (one-line caller) tcp client. Run it similarly.
4) Server echoes back whatever it receives.
5) Modify client part to send some custom message but limit collecting only five bytes at a time.

------- http server section starts ---------
6) Build http library. Converts stream of bytes to http request and vice versa - http response to stream of bytes. Thus it mus have two new types: - 'HttpRequest' and 'HttpResponse'.
