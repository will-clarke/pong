use geometry::line_segments::LineSegments;
use ball::Ball;
use paddle::Paddle;
use state::{State,IntersectionLineTypes};
use io::Input;
use score::Score;

// TODO: add r_paddle to board struct
pub struct Board {
    pub ball: Ball,
    // todo: remove this guy as it's just confusing
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
}

impl Board {

    pub fn new() -> Board {
        let l_paddle = Paddle::new();
        let line_segments =  LineSegments::new_top_and_bottom_guards();

        Board {
            ball: Ball::new(),
            reflective_lines: line_segments,
            l_paddle: l_paddle,
        }
    }

    pub fn update_game_state<'a>(&'a mut self, input: &'a mut Input, state: &'a mut State, score: &'a mut Score, tick_count: i32) {

        if state.shape_toggle {
            state.shape.rotate(tick_count);
        }

        // todo: refactor these toggles!
        if input.shape_toggle {
            if state.shape_toggle {
                state.shape_toggle = false;
            } else {
                state.shape_toggle = true;
            }
        }

        let intersection_line_types = if state.shape_toggle {
            IntersectionLineTypes::with_shape()
        } else {
            IntersectionLineTypes::without_shape()
        };
        state.intersection_line_types = intersection_line_types;

        self.l_paddle.update(input);
        let paddle_line_segment = self.l_paddle.line_segment();
        state.paddle_line = paddle_line_segment.clone();

        self.ball = self.ball.update_position(state, input, score);
        info!("<ball: [x: {}, y: {}]>", self.ball.current_position.x, self.ball.current_position.y);
    }

    // pub fn update_shape(&mut self) -> LineSegments {
    //     let vec_a = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 };
    //     let vec_b = Vector { x: (*ui::MAX_X / 3) as f64 * 2.0, y: (*ui::MAX_Y / 3) as f64 };
    //     let vec_c = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 * 2.0 };
    //     let a = LineSegment(vec_a, vec_b);
    //     let b = LineSegment(vec_b, vec_c);
    //     let c = LineSegment(vec_c, vec_a);
    //     LineSegments(vec!(a, b, c))
    //     // line_references.0.push(&a);
    // }

}
