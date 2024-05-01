package bgu.spl.net.impl.tftp;

import bgu.spl.net.srv.ConnectionHandler;
import bgu.spl.net.srv.Connections;

import java.nio.channels.AlreadyConnectedException;
import java.util.HashMap;

public class TftpConnections<T> implements Connections<T> {

    HashMap<Integer, ConnectionHandler<T>> activeUserHashMap = new HashMap<>();

    public synchronized boolean canConnect(int connectionId) {
        return !activeUserHashMap.containsKey(connectionId);
    }

    @Override
    public synchronized void connect(int connectionId, ConnectionHandler<T> handler) {
        if (!canConnect(connectionId))
            throw new AlreadyConnectedException();
        activeUserHashMap.put(connectionId, handler);
    }

    @Override
    public synchronized boolean send(int connectionId, T msg) {
        if (!activeUserHashMap.containsKey(connectionId))
            return false;
        activeUserHashMap.get(connectionId).send(msg);
        return true;
    }

    @Override
    public synchronized void disconnect(int connectionId) {
        activeUserHashMap.remove(connectionId);
    }

    public synchronized void bCast(T msg, int ID) {
        for (int connectionID : activeUserHashMap.keySet()) {
            activeUserHashMap.get(connectionID).send(msg);
        }
    }
}
