package bgu.spl.net.srv;

public interface Connections<T> {
    boolean canConnect(int connectionId);

    void connect(int connectionId, ConnectionHandler<T> handler);

    boolean send(int connectionId, T msg);

    void disconnect(int connectionId);

    void bCast(T msg, int ID);
}
