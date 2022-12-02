package de.hamburgchimps.krampus.twentytwentytwo;

import org.jboss.resteasy.reactive.RestPath;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;

@Path("/day")
public class SolutionResource {

    @GET
    @Path("/{dayNum}/part/{partNum}")
    @Produces(MediaType.APPLICATION_JSON)
    public Solution.Result solve(@RestPath int dayNum, @RestPath int partNum) {
        return Solution.execute(dayNum, partNum);
    }
}