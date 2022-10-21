package gotrellis

import (
	"net"
	"strconv"
	"strings"
)

type Connection struct {
	LocalAddr Address
	PeerAddr  Address
	conn      net.Conn
}

func (conn *Connection) Close() error {
	return conn.conn.Close()
}

func NewConnection(connect_to Address) (*Connection, error) {
	netConn, err := net.Dial("tcp", connect_to.Host+":"+strconv.Itoa(int(connect_to.Port)))
	if err != nil {
		return nil, err
	}

	host, portStr, _ := strings.Cut(netConn.LocalAddr().String(), ":")
	portU64, err := strconv.ParseUint(portStr, 16, 16)
	if err != nil {
		return nil, err
	}
	port := uint16(portU64)

	peerAddr := connect_to
	conn := Connection{
		LocalAddr: Address{
			Host: host,
			Port: port,
		},
		PeerAddr: peerAddr,
		conn:     netConn,
	}
	return &conn, nil
}

func newConnection_withNetConn(netConn net.Conn) (*Connection, error) {
	host, portStr, _ := strings.Cut(netConn.LocalAddr().String(), ":")
	portU64, err := strconv.ParseUint(portStr, 16, 16)
	if err != nil {
		return nil, err
	}
	port := uint16(portU64)
	localAddr := Address{
		Host: host,
		Port: port,
	}

	host, portStr, _ = strings.Cut(netConn.RemoteAddr().String(), ":")
	portU64, err = strconv.ParseUint(portStr, 16, 16)
	if err != nil {
		return nil, err
	}
	port = uint16(portU64)
	peerAddr := Address{
		Host: host,
		Port: port,
	}

	conn := Connection{
		LocalAddr: localAddr,
		PeerAddr:  peerAddr,
		conn:      netConn,
	}
	return &conn, nil
}
