package bgu.spl.net.impl.tftp;

import bgu.spl.net.Util;
import bgu.spl.net.api.BidiMessagingProtocol;
import bgu.spl.net.srv.Connections;
import com.sun.org.apache.xml.internal.security.algorithms.implementations.IntegrityHmac;
import com.sun.tools.jdi.Packet;

import java.net.DatagramPacket;
import java.nio.charset.StandardCharsets;
import java.util.Hashtable;
import java.util.Vector;

public class TftpProtocol implements BidiMessagingProtocol<byte[]> {

    private boolean shouldTerminate = false;
    private Connections<byte[]> connections;
    private int ownerId;
    private Vector<String> userNames = new Vector<>();


    @Override
    public void start(int connectionId, Connections<byte[]> connections) {
        this.connections = connections;
        this.ownerId = connectionId;
    }

    @Override
    public void process(byte[] message) {
        byte type = message[1];
        switch (type) {
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
            case 6:
            case 7:
                //LogRQ(message);
            case 8:
            case 9:
            case 0x10:
        }
    }

    @Override
    public boolean shouldTerminate() {
        shouldTerminate = true;
        // TODO implement this
        throw new UnsupportedOperationException("Unimplemented method 'shouldTerminate'");
    }

/*    private Packet LogRQ(byte[] message) {
        byte[] name = new byte[message.length - 3];
        Packet packet = new DatagramPacket();
        System.arraycopy(message, 2, name, 0, name.length);
        String userName = new String(Util.hexToBytes(name), StandardCharsets.UTF_8);
        if (userNames.contains(userName))
            ;
        else {
            userNames.add(userName);
            //ACK
        }
        return Packet;
    }*/


}
