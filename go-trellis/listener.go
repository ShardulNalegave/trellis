package gotrellis

import (
	"net"
	"strconv"
)

type Listener struct {
	LocalAddr Address
	lis       net.Listener
}

func (lis *Listener) Close() error {
	return lis.lis.Close()
}

func (lis *Listener) Accept() (*Connection, error) {
	netConn, err := lis.lis.Accept()
	if err != nil {
		return nil, err
	}

	conn, err := newConnection_withNetConn(netConn)
	if err != nil {
		return nil, err
	}

	return conn, nil
}

func NewListener(listen_at Address) (*Listener, error) {
	netLis, err := net.Listen("tcp", listen_at.Host+":"+strconv.Itoa(int(listen_at.Port)))
	if err != nil {
		return nil, err
	}

	lis := Listener{
		LocalAddr: listen_at,
		lis:       netLis,
	}
	return &lis, nil
}
