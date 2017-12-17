package supermax

import (
	"encoding/json"
	"log"
	"net/http"
	"net/url"
	"path"

	"github.com/natumn/gentleman/context"
)

//Client has request and response etc.
type Client struct {
	URL     *url.URL
	Context *context.Context

	Logger *log.Logger
}

// New return Client instance.
func New() (*Client, error) {
	return &Client{
		Context: context.New(),
		Logger:  log.New(),
	}
}

// Request return newReqeust instance.
func Request() {
	return newRequest()
}

func (c *Client) newRequest(ctx context.Context, spath string) *Request {
	u := *c.URL
	u.Path = path.Join(c.URL.Path, spath)
	if err != nil {
		return nil, err
	}

	req = req.WithContext(ctx)
	req.Header.Set("User-Agent", UserAgent)

	return req, nil
}

func decodeBody(response *http.Response, out interface{}) error {
	defer resp.Body.Close()
	decoder := json.NewDecoder(resp.Body)
	return decoder.Decode(out)
}
