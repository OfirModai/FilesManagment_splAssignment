package bgu.spl.net.srv;

import java.nio.channels.AlreadyConnectedException;
import java.util.HashMap;

public class BaseConnections<T> implements Connections<T> {

    HashMap<Integer, ConnectionHandler<T>> activeUserHashMap = new HashMap<>();

    public boolean canConnect(int connectionId) {
        return activeUserHashMap.containsKey(connectionId);
    }

    @Override
    public void connect(int connectionId, ConnectionHandler<T> handler) {
        if (!canConnect(connectionId))
            throw new AlreadyConnectedException();
        activeUserHashMap.put(connectionId, handler);
    }

    @Override
    public boolean send(int connectionId, T msg) {
        if (!activeUserHashMap.containsKey(connectionId))
            return false;
        activeUserHashMap.get(connectionId).send(msg);
        return true;
    }

    @Override
    public void disconnect(int connectionId) {
        activeUserHashMap.remove(connectionId);
    }
}
